import { useState } from "react";
import loomLogo from "./assets/loom.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Timeline from "./components/Timeline";
import type { TimelineItem } from "./types";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  const timelineItems: TimelineItem[] = [
    {
      id: "1",
      date: new Date("2023-05-01"),
      type: "sensor",
      data: { temperature: 22, humidity: 45 },
    },
    {
      id: "2",
      date: new Date("2023-05-02"),
      type: "note",
      data: { content: "Important meeting notes" },
    },
    {
      id: "3",
      date: new Date("2023-05-03"),
      type: "image",
      data: { url: "/placeholder.svg?height=200&width=300", caption: "Project screenshot" },
    },
    {
      id: "4",
      date: new Date("2023-05-04"),
      type: "link",
      data: { url: "https://example.com", title: "Interesting article" },
    },
    {
      id: "5",
      date: new Date("2023-05-05"),
      type: "note",
      data: { content: "Another important note" },
    },
    {
      id: "6",
      date: new Date("2023-05-06"),
      type: "image",
      data: { url: "/placeholder.svg?height=200&width=300", caption: "Project screenshot" },
    },
    {
      id: "7",
      date: new Date("2023-05-07"),
      type: "note",
      data: { content: "Another important note" },
    },
    {
      id: "8",
      date: new Date("2023-05-08"),
      type: "image",
      data: { url: "/placeholder.svg?height=200&width=300", caption: "Project screenshot" },
    },
    
    
  ];

  return (
    <div className="min-h-screen bg-gray-100 p-8">
      <div className="flex items-center justify-left gap-4 mb-8">
        <img src={loomLogo} className="w-16 h-16" alt="Loom logo" />
        <h1 className="text-3xl font-bold">Loom</h1>
      </div>
      <div className="grid grid-cols-2 gap-8">
        <div className="col-span-1 mt-10">
          <Timeline items={timelineItems} />
        </div>
        <div className="col-span-1 grid grid-rows-2 gap-8 content-start">
          <div className="flex flex-col gap-8 content-start">
            <div className="bg-white rounded-lg shadow p-4 h-[50vh]">
              {/* Top right area */}
            </div>
            <div className="bg-white rounded-lg shadow p-4 h-[50vh]">
              {/* Bottom right area */}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
