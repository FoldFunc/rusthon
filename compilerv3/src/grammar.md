$$
\begin{align}
[\text{program}] &\to [\text{function}]^+ \\
[\text{function}] &\to [\text{stmt}]^+ \\
[\text{stmt}] &\to
\begin{cases}
\text{return \text{[expr]}}; \\
\text{let ident = [\text{expr}]}; \\
\text{ident = [\text{expr}]}; \\
\end{cases} \\
[\text{term}] &\to
\begin{cases}
[\text{int\_lit}] \\
[\text{ident}]
\end{cases} \\
[\text{expr}] &\to
\begin{cases}
[\text{term}] \\
[\text{binary\_expr}] \\
\end{cases} \\
[\text{binary\_expr}] &\to
\begin{cases}
[\text{expr}] &\text{+}& [\text{expr}] \text{pred: } 1 \\
[\text{expr}] &\text{-}& [\text{expr}] \text{pred: } 1 \\
[\text{expr}] &\text{/}& [\text{expr}] \text{pred: } 2 \\
[\text{expr}] &\text{*}& [\text{expr}] \text{pred: } 2 \\
[\text{expr}] &\text{==}& [\text{expr}] \text{pred: } 3 \\
[\text{expr}] &\text{>=}& [\text{expr}] \text{pred: } 3 \\
[\text{expr}] &\text{<=}& [\text{expr}] \text{pred: } 3 \\
\end{cases}
\end{align}
$$
