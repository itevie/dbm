import React, { useState } from "react";
import { useMakerSelector } from "../reduxStore";
import BotController from "../components/BotController";
import Input from "../components/Input";
import { useDispatch } from "react-redux";
import { asyncBasicInputModel } from "../components/models/modelTemplates";
import { invoke } from "@tauri-apps/api";
import { Bot } from "../types/structures";
import { addBot } from "../stores/bots";
import HiddenDiv from "../components/HiddenDiv";
import { setSave } from "../App";

export default function BotSettings() {
    const dispatch = useDispatch();
    const bots = useMakerSelector(state => state.bots);
    const settings = useMakerSelector(state => state.settings);
    const [updatedSettings, setUpdatedSettings] = useState<{ [key: string]: HTMLInputElement }>({});

    async function setBotName() {
        let name = await asyncBasicInputModel(`Change ${bots[settings.current_bot || -1].name}'s name`, "", "New Name");
        let bot = await invoke<Bot>("set_bot_name", { id: settings.current_bot, name });
        dispatch(addBot(bot));
    }

    async function setBotDescription() {
        let description = await asyncBasicInputModel(`Change ${bots[settings.current_bot || -1].name}'s description`, "", "New Description");
        let bot = await invoke<Bot>("set_bot_description", { id: settings.current_bot, description });
        dispatch(addBot(bot));
    }

    function setUpdate(name: string, value: HTMLInputElement) {
        setUpdatedSettings({ ...updatedSettings, [name]: value });
        setSave(async () => {
            for (const key in updatedSettings) {
                switch (key) {
                    case "prefix":
                        dispatch(addBot(await invoke<Bot>("set_bot_prefix", { id: settings.current_bot, prefix: updatedSettings[key].value })));
                    case "token":
                        dispatch(addBot(await invoke<Bot>("set_bot_token", { id: settings.current_bot, token: updatedSettings[key].value })));
                }
            }
            setUpdatedSettings({});
        });
    }

    return (
        <>
            {!settings.current_bot ? <></>
                :
                <>
                    <BotController onSelect={() => { }} bot={settings.current_bot} />
                    <h1>Details</h1>
                    <p>Name: <b onClick={setBotName} className="editable">{bots[settings.current_bot].name}</b></p>
                    <p>Desciption: <b onClick={setBotDescription} className="editable">{bots[settings.current_bot].description}</b></p>
                    <h1>Basic Settings</h1>
                    <label>Default Prefix:</label>
                    <Input onKeyUp={v => setUpdate("prefix", (v.target as HTMLInputElement))} className="jumbo" defaultValue={bots[settings.current_bot].prefix} />
                    <HiddenDiv title="Token">
                        <label>Token</label>
                        <Input onKeyUp={v => setUpdate("token", (v.target as HTMLInputElement))} className="jumbo" defaultValue={bots[settings.current_bot].token} />
                    </HiddenDiv>
                </>
            }
        </>
    )
}