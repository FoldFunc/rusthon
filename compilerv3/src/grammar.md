$$
\begin{align}
[\text{program}] &\to [\text{function}]^+ \\
[\text{function}] &\to [\text{stmt}]^+ \\
[\text{stmt}] &\to
\begin{cases}
\text{return \text{[expr]}}; \\
\text{let ident = [\text{expr}]}; \\
\end{cases} \\
[\text{expr}] &\to
\begin{cases}
[\text{int\_lit}] \\
[\text{ident}] \\
\end{cases} \\
\end{align}
$$
