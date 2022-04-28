# Templates

Build file: `pdflatex main; biber main; pdflatex main; pdflatex main;`

## Citation

Add quotes with `\q{Text}`.

## Acronym

* `\newacronym{ast}{AST}{Abstract syntax tree}`
    * `\acrshort{}` -> AST
    * `\acrlong{}` -> Abstract syntax tree
    * `\acrfull{}` -> Abstract syntax tree (AST)

## Figure

```latex
\begin{figure}[H]
    \centering
    \includegraphics[width=1\columnwidth]{res/example.png}
    \caption[Short name]{Looooooong name}
    \scriptsize Possible reference
\end{figure}
```

## Wrap text around a figure:

```latex
\setlength{\columnsep}{0.6cm} % Width of the image column
\begin{wrapfigure}{R}{0.4\columnwidth}
    \begin{center}
        \includegraphics[width=0.4\columnwidth]{res/example.png}
    \caption[Short name]{Looooooong name}
    \scriptsize Possible reference
    \end{center}
\end{wrapfigure}

The text that will be wrapped
```

## Tables

```latex
\begin{table}[H]
    % Use `L{?cm}`, `R{?cm}` or `C{?cm}` for fixed size columns (automatically wraps text)
    % Use min one `X` to fill the set width
    \begin{tabularx}{\textwidth}{|L{4cm}|X|c|}
        \hline
        1 & 2 & 3 \hline
    \end{tabularx}
\end{table}
```

## Kill *Hurenkinder* und *Schusterjungen*

```latex
% Hurenkinder und Schusterjungen verhindern
\clubpenalty10000
\widowpenalty10000
\displaywidowpenalty=10000
```
