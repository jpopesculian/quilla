import { useMemo } from "react";
import CodeMirror from "@uiw/react-codemirror";
import { StreamLanguage } from "@codemirror/language";
import { elm } from "@codemirror/legacy-modes/mode/elm";
import { vim } from "@replit/codemirror-vim";
import { Bar, BarChart, XAxis, YAxis } from "recharts";
import { ModeToggle } from "@/components/mode-toggle";
import { useVimStore } from "@/stores/vim";
import { useCodeStore } from "@/stores/code";
import { buttonVariants } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { shadcnTheme } from "@/lib/codemirror-theme";
import { quillaLinter } from "@/lib/quilla-lint";
import {
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
  type ChartConfig,
} from "@/components/ui/chart";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";

const chartConfig = {
  count: {
    label: "Count",
    color: "var(--primary)",
  },
} satisfies ChartConfig;

function App() {
  const code = useCodeStore((s) => s.code);
  const setCode = useCodeStore((s) => s.setCode);
  const drawing = useCodeStore((s) => s.drawing);
  const samples = useCodeStore((s) => s.samples);
  const vimEnabled = useVimStore((s) => s.enabled);
  const toggleVim = useVimStore((s) => s.toggle);

  const chartData = useMemo(() => {
    if (!samples) return [];
    return Array.from(samples.entries())
      .map(([state, count]) => ({ state, count }))
      .sort((a, b) => a.state.localeCompare(b.state));
  }, [samples]);

  const extensions = useMemo(
    () => [
      ...(vimEnabled ? [vim()] : []),
      StreamLanguage.define(elm),
      ...shadcnTheme,
      quillaLinter,
    ],
    [vimEnabled],
  );

  return (
    <div className="flex h-screen flex-col bg-background text-foreground">
      <header className="flex items-center justify-between border-b px-4 py-2">
        <span className="text-sm font-medium">Quilla</span>
        <div className="flex items-center gap-2">
          <Dialog>
            <DialogTrigger
              className={buttonVariants({ variant: "outline", size: "icon" })}
              title="Help"
            >
              <span className="text-xs font-bold">?</span>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Circuit Instructions</DialogTitle>
              </DialogHeader>
              <div className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 font-mono text-sm">
                <span className="text-muted-foreground">h target</span>
                <span>Hadamard gate</span>
                <span className="text-muted-foreground">i target</span>
                <span>Identity gate</span>
                <span className="text-muted-foreground">x target</span>
                <span>Pauli-X gate</span>
                <span className="text-muted-foreground">y target</span>
                <span>Pauli-Y gate</span>
                <span className="text-muted-foreground">z target</span>
                <span>Pauli-Z gate</span>
                <span className="text-muted-foreground">s target</span>
                <span>S gate (phase)</span>
                <span className="text-muted-foreground">sdg target</span>
                <span>S-dagger gate</span>
                <span className="text-muted-foreground">t target</span>
                <span>T gate (pi/8)</span>
                <span className="text-muted-foreground">tdg target</span>
                <span>T-dagger gate</span>
                <span className="text-muted-foreground">cx ctrl target</span>
                <span>Controlled-X (CNOT)</span>
                <span className="text-muted-foreground">cy ctrl target</span>
                <span>Controlled-Y</span>
                <span className="text-muted-foreground">cz ctrl target</span>
                <span>Controlled-Z</span>
                <span className="text-muted-foreground">swap q1 q2</span>
                <span>Swap gate</span>
                <span className="text-muted-foreground">rx theta target</span>
                <span>X-rotation</span>
                <span className="text-muted-foreground">ry theta target</span>
                <span>Y-rotation</span>
                <span className="text-muted-foreground">rz theta target</span>
                <span>Z-rotation</span>
                <span className="text-muted-foreground">m qbit cbit</span>
                <span>Measure qubit to classical bit</span>
              </div>
            </DialogContent>
          </Dialog>
          <button
            onClick={toggleVim}
            className={buttonVariants({
              variant: vimEnabled ? "default" : "outline",
              size: "icon",
            })}
            title="Toggle Vim mode"
          >
            <span className="text-xs font-bold">Vi</span>
          </button>
          <ModeToggle />
        </div>
      </header>
      <ResizablePanelGroup orientation="vertical" className="flex-1">
        <ResizablePanel defaultSize={60}>
          <ResizablePanelGroup orientation="horizontal">
            <ResizablePanel defaultSize={50}>
              <CodeMirror
                value={code}
                onChange={setCode}
                extensions={extensions}
                theme="none"
                autoFocus
                height="100%"
                className="h-full"
              />
            </ResizablePanel>
            <ResizableHandle withHandle />
            <ResizablePanel defaultSize={50}>
              <div className="flex h-full items-center justify-center p-4">
                {chartData.length > 0 ? (
                  <ChartContainer
                    config={chartConfig}
                    className="h-full w-full"
                  >
                    <BarChart data={chartData}>
                      <XAxis
                        dataKey="state"
                        tickLine={false}
                        axisLine={false}
                      />
                      <YAxis tickLine={false} axisLine={false} />
                      <ChartTooltip content={<ChartTooltipContent />} />
                      <Bar
                        dataKey="count"
                        fill="var(--color-count)"
                        radius={4}
                      />
                    </BarChart>
                  </ChartContainer>
                ) : (
                  <span className="text-sm text-muted-foreground">
                    No samples
                  </span>
                )}
              </div>
            </ResizablePanel>
          </ResizablePanelGroup>
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel defaultSize={40}>
          <div className="flex h-full items-center justify-center overflow-auto p-4">
            {drawing ? (
              <pre className="font-mono text-sm leading-none">{drawing}</pre>
            ) : (
              <span className="text-sm text-muted-foreground">No circuit</span>
            )}
          </div>
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}

export default App;
