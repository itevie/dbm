import React, { useEffect, useRef, useState } from "react";
import Container from "../components/Container";
import { CodePiece, Command } from "../types/structures";
import { invoke } from "@tauri-apps/api";
import { useMakerSelector, useSettingsSelector } from "../reduxStore";
import { useDispatch } from "react-redux";
import { addCommand } from "../stores/commands";
import Button from "../components/Button";
import HeaderText from "../components/HeaderText";
import { Editor } from "@monaco-editor/react";
import { asyncBasicInputModel } from "../components/models/modelTemplates";

export default function CommandsPage() {
    const dispatch = useDispatch();
    const commands = useMakerSelector(state => state.commands);
    const settings = useSettingsSelector(state => state.settings);
    const [currentCommand, setCurrentCommand] = useState<number | null>(null);
    const [currentCode, setCurrentCode] = useState<string>("");
    const editorRef = useRef<any>(null);

    useEffect(() => {
        (async () => {
            if (!settings.current_bot) return;
            let commands = await invoke("get_all_commands", { botId: settings.current_bot }) as Command[];
            for (const command of commands)
                dispatch(addCommand(command));
        })();
    }, [settings]);

    async function createCommand() {
        let name = await asyncBasicInputModel("Create Command", "", "Name");
        if (!settings.current_bot || !name) return;
        const command = await invoke("create_command", { botId: settings.current_bot, name }) as Command;
        dispatch(addCommand(command));
    }

    async function loadCommand(id: number) {
        let command = commands[id];
        if (!command.code_id) {
            let codePiece = await invoke<CodePiece>("create_code_piece", { commandId: command.id });
            command = { ...command, code_id: codePiece.id };
            dispatch(addCommand(command));
        }

        let codePiece = await invoke<CodePiece>("get_code_piece", { id: command.code_id });
        setCurrentCode(codePiece.code);
        setCurrentCommand(command.id);
        editorRef.current.getModel().setValue(codePiece.code);
    }

    async function changeCommandName(id: number) {
        let command = commands[id];
        let newName = await asyncBasicInputModel(`Change ${command.name}'s Name`, null, "New Name");
        let newCommand = await invoke<Command>("set_command_name", { id: command.id, name: newName });
        dispatch(addCommand(newCommand));
    }

    async function save() {
        let value = editorRef.current.getModel().getValue();
        await invoke("set_code_piece", { id: commands[currentCommand as number].code_id, code: value })
    }

    function onMount(e: any) {
        editorRef.current = e;
    }

    return (
        <>
            <div className="flex-row">
                <Container>
                    <Button className="jumbo" onClick={createCommand}>New</Button><br />
                    {
                        Object.keys(commands).filter(x => commands[x].bot_id === settings.current_bot).map(x => <>
                            <label className="container-item" onClick={() => loadCommand(commands[x].id)}>{commands[x].name}</label>
                        </>)
                    }
                </Container>
                <div className="flex-1">
                    {currentCommand &&
                        <>
                            <HeaderText onClick={() => changeCommandName(currentCommand)} className="editable">{commands[currentCommand].name}</HeaderText>
                            <i className="editable">Some kind of editable description goes here</i>
                            <Editor theme="vs-dark" onMount={e => onMount(e)} height={400} value={currentCode} />
                            <Button className="jumbo" onClick={save}>Save</Button>
                        </>
                    }
                </div>
            </div>
        </>
    )
}