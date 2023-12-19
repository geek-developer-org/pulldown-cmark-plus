cat github-markdown.css custom-style.css>.temp
cleancss -o github-markdown.min.css .temp

cat github-markdown-light.css custom-style-light.css>.temp
cleancss -o github-markdown-light.min.css .temp

cat github-markdown-dark.css custom-style-dark.css>.temp
cleancss -o github-markdown-dark.min.css .temp

rm .temp
