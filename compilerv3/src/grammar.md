$$
\begin{align}
[\text{program}] &\to [\text{function}]^+ \\
[\text{function}] &\to [\text{stmt}]^+ \\
[\text{stmt}] &\to
\begin{cases}
\text{return \text{[expr]}}; \\
\text{let ident = [\text{expr}]}; \\
\text{ident = [\text{expr}]}; \\
\text{ident += [\text{expr}]}; \\
\text{ident -= [\text{expr}]}; \\
\text{ident *= [\text{expr}]}; \\
\text{ident /= [\text{expr}]}; \\
\end{cases} \\
[\text{term}] &\to
\begin{cases}
[\text{int\_lit}] \\
[\text{ident}] \\
([\text{expr}]) \\
\end{cases} \\
[\text{expr}] &\to
\begin{cases}
[\text{term}] \\
[\text{binary\_expr}] \\
\end{cases} \\
[\text{signs}] &\to
\begin{cases}
[\text{+}] &\text{pred: } 1 \\
[\text{-}] &\text{pred: } 1 \\
[\text{*}] &\text{pred: } 2  \\
[\text{/}] &\text{pred: } 2  \\
[\text{<=}] &\text{pred: } 0  \\
[\text{>=}] &\text{pred: } 0  \\
\end{cases} \\
[\text{binary\_expr}] &\to
\begin{cases}
[\text{expr}] \space [\text{sign}] \space[\text{expr}] \\
\end{cases}
\end{align}
$$
