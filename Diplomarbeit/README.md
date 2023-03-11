# Installationshinweise

1. LaTeX Workshop Installieren (VsCode Extension)
2. TexLive installieren
3. Folgenden key zu `settings.json` hinzufügen:

```json
  "latex-workshop.latex.tools": [
    {
      "name": "latexmk",
      "command": "latexmk",
      "args": [
        "--shell-escape",
        "-synctex=1",
        "-interaction=nonstopmode",
        "-file-line-error",
        "-pdf",
        "-outdir=%OUTDIR%",
        "%DOC%"
      ]
    },
    {
      "name": "pdflatex",
      "command": "pdflatex",
      "args": [
        "--shell-escape",
        "-synctex=1",
        "-interaction=nonstopmode",
        "-file-line-error",
        "%DOC%"
      ]
    },
    {
      "name": "bibtex",
      "command": "bibtex",
      "args": ["%DOCFILE%"]
    }
  ],
```

4. Python Installieren
5. Python Installationspfad zur `Path` Systemvariable hinzufügen (insofen dieser nicht bei der installation hinzugefügt wurde)
6. Pygments Bibliothek Installieren (`pip install pygments --force-reinstall`)

# Häufige Fehlerquellen

1. Komma Fehler in der `literatur.bib` Datei:
   Fehlendes Komma führt dazu, dass der gesammte Build scheitert
