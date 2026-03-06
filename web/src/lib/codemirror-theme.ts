import { EditorView } from "@codemirror/view"
import { HighlightStyle, syntaxHighlighting } from "@codemirror/language"
import { tags } from "@lezer/highlight"

const baseTheme = EditorView.theme({
  "&": {
    backgroundColor: "var(--background)",
    color: "var(--foreground)",
    fontFamily: "var(--font-mono)",
  },
  ".cm-content": {
    caretColor: "var(--foreground)",
  },
  ".cm-cursor, .cm-dropCursor": {
    borderLeftColor: "var(--foreground)",
  },
  "&.cm-focused .cm-selectionBackground, .cm-selectionBackground, .cm-content ::selection":
    {
      backgroundColor: "var(--accent)",
    },
  ".cm-activeLine": {
    backgroundColor: "var(--accent)",
  },
  ".cm-gutters": {
    backgroundColor: "var(--background)",
    color: "var(--muted-foreground)",
    borderRight: "1px solid var(--border)",
  },
  ".cm-activeLineGutter": {
    backgroundColor: "var(--accent)",
  },
  ".cm-lineNumbers .cm-gutterElement": {
    color: "var(--muted-foreground)",
  },
  ".cm-diagnostic": {
    backgroundColor: "var(--popover)",
    color: "var(--popover-foreground)",
    border: "1px solid var(--border)",
    borderRadius: "var(--radius)",
    padding: "4px 8px",
  },
  ".cm-diagnostic-error": {
    borderLeftColor: "var(--destructive)",
  },
  ".cm-tooltip": {
    backgroundColor: "var(--popover)",
    color: "var(--popover-foreground)",
    border: "1px solid var(--border)",
    borderRadius: "var(--radius)",
  },
  ".cm-tooltip .cm-diagnostic": {
    border: "none",
  },
})

const highlightStyle = HighlightStyle.define([
  { tag: tags.keyword, color: "var(--syntax-keyword)" },
  { tag: [tags.name, tags.deleted, tags.character, tags.macroName], color: "var(--foreground)" },
  { tag: [tags.function(tags.variableName)], color: "var(--syntax-function)" },
  { tag: [tags.labelName], color: "var(--foreground)" },
  { tag: [tags.color, tags.constant(tags.name), tags.standard(tags.name)], color: "var(--syntax-number)" },
  { tag: [tags.definition(tags.name), tags.separator], color: "var(--foreground)" },
  { tag: [tags.typeName, tags.className, tags.changed, tags.annotation, tags.modifier, tags.self, tags.namespace], color: "var(--syntax-type)" },
  { tag: [tags.number], color: "var(--syntax-number)" },
  { tag: [tags.operator, tags.operatorKeyword], color: "var(--muted-foreground)" },
  { tag: [tags.url, tags.escape, tags.regexp, tags.link], color: "var(--syntax-function)" },
  { tag: [tags.meta, tags.comment], color: "var(--muted-foreground)" },
  { tag: tags.strong, fontWeight: "bold" },
  { tag: tags.emphasis, fontStyle: "italic" },
  { tag: tags.strikethrough, textDecoration: "line-through" },
  { tag: tags.link, textDecoration: "underline" },
  { tag: tags.heading, fontWeight: "bold", color: "var(--foreground)" },
  { tag: [tags.atom, tags.bool, tags.special(tags.variableName)], color: "var(--syntax-number)" },
  { tag: [tags.processingInstruction, tags.string, tags.inserted], color: "var(--syntax-string)" },
  { tag: tags.invalid, color: "var(--destructive)" },
])

export const shadcnTheme = [baseTheme, syntaxHighlighting(highlightStyle)]
