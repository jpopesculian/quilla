import { linter, type Diagnostic } from "@codemirror/lint"
import { useCodeStore } from "@/stores/code"

export const quillaLinter = linter((view) => {
  const parsed = useCodeStore.getState().parsed
  const diagnostics: Diagnostic[] = []
  for (const item of parsed) {
    if ("error" in item) {
      diagnostics.push({
        from: Math.min(item.span.start, view.state.doc.length),
        to: Math.min(item.span.end, view.state.doc.length),
        severity: "error",
        message: item.error,
      })
    }
  }
  return diagnostics
})
