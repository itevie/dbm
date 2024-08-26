import React, { useEffect } from "react";
import HeaderText from "../components/HeaderText";
import { useBotsSelector } from "../reduxStore";
import { invoke } from "@tauri-apps/api";
import { useDispatch } from "react-redux";
import { addBot } from "../stores/bots";
import { Bot, Settings } from "../types/structures";
import { setSettings } from "../stores/options";

export default function BotList() {
    const dispatch = useDispatch();
    const bots = useBotsSelector(state => state.bots);

    useEffect(() => {
        (async () => {
            let bots = await invoke("get_all_bots") as Bot[];
            console.log(bots, 2);
            for (const bot of bots)
                dispatch(addBot(bot));
        })();
    }, []);

    async function selectBot(id: number) {
        let settings = await invoke("set_selected_bot", { botId: id }) as Settings;
        dispatch(setSettings(settings));
    }

    return (
        <>
            <HeaderText>Bot List</HeaderText>
            {
                Object.keys(bots).map(x => <>
                    {bots[x].name}
                    <button onClick={() => selectBot(bots[x].id)}>Select</button>
                    <br />
                </>)
            }
        </>
    );
}