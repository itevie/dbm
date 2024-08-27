import { useMakerSelector } from "../reduxStore";
import React from "react";
import Button from "./Button";
import { invoke } from "@tauri-apps/api";

export default function BotController({ bot, onSelect }: { bot: number, onSelect: (what: number) => void }) {
    const bots = useMakerSelector(state => state.bots);
    const runningBots = useMakerSelector(state => state.runningBots);
    const settings = useMakerSelector(state => state.settings);

    async function startBot(id: number) {
        await invoke("run_bot", { id });
    }

    async function stopBot(id: number) {
        await invoke("stop_bot", { id });
    }

    return (
        <div
            onDoubleClick={() => onSelect(bot)}
            className={`container ${settings?.current_bot === bot ? "focused" : ""}`}
            style={{ display: "flex", justifyContent: "space-between", alignItems: "center", gap: "7px", flex: "spread-around" }}
        >
            <b>{bots[bot].name}</b>
            <div style={{ display: "flex", gap: "10px" }}>
                {runningBots.includes(bot)
                    ? <Button onClick={() => stopBot(bot)} className="error">End</Button>
                    : <Button onClick={() => startBot(bot)} className="success">Start</Button>
                }
                <Button onClick={() => onSelect(bot)} disabled={settings.current_bot === bot}>Select</Button>
            </div>
        </div >
    )
}