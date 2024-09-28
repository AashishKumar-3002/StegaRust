"use client"

import { useState } from "react"
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs"
import EncodeSection from "./Encode"
import DecodeSection from "./Decode"
import PrintSection from "./Print"
import RemoveSection from "./Remove"
import StatusSection from "./Status"

export function StegaRustUi() {
  const [activeTab, setActiveTab] = useState("encode")

  return (
    <div className="flex flex-col min-h-screen bg-slate-100">
      {/* Navbar */}
      <nav className="bg-[#cc5500] text-white p-4">
        <div className="container mx-auto flex justify-between items-center">
          <h1 className="text-2xl font-bold">StegaRust</h1>
          <ul className="flex space-x-4">
            {["Home", "Encode", "Decode", "Print", "Remove", "Status"].map((item) => (
              <li key={item}>
                <a
                  href="#"
                  className="hover:underline"
                  onClick={(e) => {
                    e.preventDefault()
                    setActiveTab(item.toLowerCase())
                  }}
                >
                  {item}
                </a>
              </li>
            ))}
          </ul>
        </div>
      </nav>

      {/* Main Content */}
      <main className="flex-grow container mx-auto mt-8 p-4">
        <Tabs value={activeTab} onValueChange={setActiveTab}>
          <TabsList className="grid w-full grid-cols-5 mb-8">
            <TabsTrigger value="encode">Encode</TabsTrigger>
            <TabsTrigger value="decode">Decode</TabsTrigger>
            <TabsTrigger value="print">Print</TabsTrigger>
            <TabsTrigger value="remove">Remove</TabsTrigger>
            <TabsTrigger value="status">Status</TabsTrigger>
          </TabsList>
          <EncodeSection />
          <DecodeSection />
          <PrintSection />
          <RemoveSection />
          <StatusSection />
        </Tabs>
      </main>

      {/* Footer */}
      <footer className="bg-[#2f4f4f] text-white p-4 mt-8">
        <div className="container mx-auto text-center">
          <div className="flex justify-center items-center space-x-4">
            <span className="font-bold">Developed by Aashish Kumar</span>
            <span className="font-bold">|</span>
            <a className="font-bold hover:underline" href="https://github.com/AashishKumar-3002/StegaRust/tree/Master_APIs">
              GitHub Repository
            </a>
            <span className="font-bold">|</span>
            <span className="font-bold">Version 1.0.0</span>
          </div>
        </div>
      </footer>
    </div>
  )
}