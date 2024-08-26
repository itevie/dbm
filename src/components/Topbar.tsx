import { invoke } from "@tauri-apps/api";
import React, { useEffect, useState } from "react";
import { useBotsSelector, useSettingsSelector } from "../reduxStore";

export default function Topbar({ page }: { page: string }) {
    const settings = useSettingsSelector(state => state.settings);
    const bots = useBotsSelector(state => state.bots);

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
                <label>{page}</label>
            </div>
            <div className="topbar-right">
                <button onClick={stopBot}>Stop</button>
                <button onClick={startBot}>Start</button>
                <label>{settings.current_bot ? bots[settings.current_bot]?.name : "No selected bot"}</label>
            </div>
        </div>
    );
}