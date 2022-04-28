// [...] Lines 1 - 14
// Not part of the `clippy::vec_init_then_push` reimplementation

const DB_URL: &str = "http://neo4j:pw-clippy-query@localhost:7474/db/data";
const VEC_INIT_THEN_PUSH_QUERY: &str = r#"
MATCH
    (assign {from_expansion: false}),
    (assign)-[:Child {index: 0}]->(var),
    (assign)-[:Child {index: 1}]->(init_call:Call)-[:Child]->(init:Path) 
WHERE 
    (var.name = "Pat" OR var.name = "Path")
    AND (assign.name = "Local" OR assign.name = "Assign")
    AND (init.path CONTAINS 'new' 
        OR init.path CONTAINS 'with_capacity'
        OR init.path CONTAINS 'default')

MATCH
    (scope)-[assign_edge:Child]->(assign),
    (scope)-[next_edge:Child]->(method:MethodCall)
WHERE
    next_edge.index = assign_edge.index + 1
    AND method.ident = "push"

return assign.hir_id, method.hir_id
"#;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Example
    /// ```rust
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// ```
    pub GRAPH_QUERY_LINTER,
    nursery,
    "default lint description"
}

declare_lint_pass!(GraphQueryLinter => [GRAPH_QUERY_LINTER]);

impl LateLintPass<'_> for GraphQueryLinter {
    // [...] Lines 60 - 66
    // Not part of the `clippy::vec_init_then_push` reimplementation
    fn check_fn(/* [...] */) {
        // [...] Lines 76 - 86
        // Not part of the `clippy::vec_init_then_push` reimplementation
        exec_query_post(cx)
    }
}

fn exec_query_post(cx: &LateContext<'tcx>) {
    let graph = GraphClient::connect(DB_URL).unwrap();
    let result = match graph.exec(VEC_INIT_THEN_PUSH_QUERY) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Query Err: {:#?}", err);
            return;
        },
    };

    for row in result.rows() {
        if let Err(err) = process_vec_init_then_push_row(cx, row) {
            eprintln!("Row Err: {:#?}", err);
        }
    }
}

fn process_vec_init_then_push_row(cx: &LateContext<'tcx>, row: Row<'_>) -> Result<(), GraphError> {
    let assign_id = deserialize_hir_id(row.get("assign.hir_id")?);
    let map = &cx.tcx.hir();
    let local_id;
    let init_expr;
    let mut has_let = false;
    let ident_span;
    let err_span;
    match map.get(assign_id) {
        hir::Node::Local(local) if let Some(init) = local.init => {
            local_id = local.pat.hir_id;
            init_expr = init;
            ident_span = local.pat.span;
            err_span = local.span;
            has_let = true;
        }
        hir::Node::Expr(expr) if let hir::ExprKind::Assign(path, init, _) = &expr.kind => {
            local_id = path_to_local(path).unwrap();
            init_expr = init;
            ident_span = path.span;
            err_span = expr.span;
        }
        node => {
            eprintln!("Unexpected node: {:#?}", node);
            return Ok(());
        }
    }

    // Checking the type
    // A proper lint implementation would go further then just checking the type but a proper
    // query based implementation might handle the graph representation better
    let ty = cx.typeck_results().expr_ty(init_expr);
    if !is_type_diagnostic_item(cx, ty, sym::Vec) {
        return Ok(());
    }

    let method_id = deserialize_hir_id(row.get("method.hir_id")?);
    let push_expr = map.expect_expr(method_id);
    if let hir::ExprKind::MethodCall(_, _, [self_expr, _push_arg], _) = push_expr.kind {
        if path_to_local_id(self_expr, local_id) {
            let mut s = if has_let { String::from("let ") } else { String::new() };
            s.push_str(&snippet(cx, ident_span, ".."));
            s.push_str(" = vec![..];");

            span_lint_and_sugg(
                cx,
                GRAPH_QUERY_LINTER,
                err_span.to(push_expr.span),
                "calls to `push` immediately after creation",
                "consider using the `vec![]` macro",
                s,
                Applicability::HasPlaceholders,
            );
        }
    }

    Ok(())
}

// [...] Lines 166 - 749
// Not part of the `clippy::vec_init_then_push` reimplementation
