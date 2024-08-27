import { invoke } from "@tauri-apps/api";
import React, { useEffect, useState } from "react";
import { useBotsSelector, useMakerSelector, useSettingsSelector } from "../reduxStore";
import Button from "./Button";

export default function Topbar({ page }: { page: string }) {
    const settings = useSettingsSelector(state => state.settings);
    const bots = useBotsSelector(state => state.bots);
    const runningBots = useMakerSelector(state => state.runningBots);

    async function startBot() {
        if (!settings.current_bot) return;
        await invoke("run_bot", { id: bots[settings.current_bot]?.id });
    }

    async function stopBot() {
        if (!settings.current_bot) return;
        await invoke("stop_bot", { id: bots[settings.current_bot]?.id });
    }

    return (
        <div className="topbar">
            <div className="topbar-left">
                <b>{page}</b>
            </div>
            <div className="topbar-right">
                {runningBots.includes(settings.current_bot ?? -1)
                    ? <Button type="error" onClick={stopBot}>Stop</Button>
                    : <Button type="success" onClick={startBot}>Start</Button>
                }
                <b>{settings.current_bot ? bots[settings.current_bot]?.name : "No selected bot"}</b>
            </div>
        </div >
    );
}