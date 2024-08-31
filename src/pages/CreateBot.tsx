import React, { useRef, useState } from "react";
import HeaderText from "../components/HeaderText";
import Button from "../components/Button";
import { invoke } from "@tauri-apps/api";
import { setSettings } from "../stores/options";
import { useDispatch } from "react-redux";
import { addBot } from "../stores/bots";
import { Bot } from "../types/structures";
import Input from "../components/Input";
import { BlocklyWorkspace } from "react-blockly";
import * as Blockly from "blockly";

Blockly.Blocks['main'] = {
    init: function () {
        this.jsonInit({
            message0: "When command is ran",
            message1: "Do %1",
            args1: [
                {
                    "type": "input_statement",
                    "name": "do"
                }
            ],
            deletable: false,
            moveable: false,
            editable: false,
            colour: 260
        });

        this.setDeletable(false);
    }
}

export default function CreateBot() {
    const dispatch = useDispatch();
    const nameRef = useRef<HTMLInputElement>(null);
    const tokenRef = useRef<HTMLInputElement>(null);
    const [test, setTest] = useState<string>("");


    async function createBot() {
        let name = nameRef.current?.value;
        let token = tokenRef.current?.value;

        console.log(name, token);

        let result = await invoke("create_bot", { name: nameRef.current?.value, token: tokenRef.current?.value }) as Bot;
        dispatch(addBot(result));
        dispatch(setSettings({ current_bot: result.id }));
    }

    const initialXml =
        '<xml xmlns="http://www.w3.org/1999/xhtml"><block type="main" x="70" y="30"></block></xml>';
    const toolboxCategories = {
        kind: "categoryToolbox",
        contents: [
            {
                kind: "category",
                name: "Blocks",
                contents: [
                    {
                        type: "main",
                        kind: "block",
                        "nextStatement": null,
                    }
                ]
            },
            {
                kind: "category",
                name: "Logic",
                colour: "#5C81A6",
                contents: [
                    {
                        'kind': 'block',
                        'type': 'logic_boolean'
                    },
                    {
                        'kind': 'block',
                        'type': 'logic_null'
                    },
                    {
                        kind: "block",
                        type: "controls_if",
                    },
                    {
                        kind: "block",
                        type: "logic_compare",
                    },
                ],
            },
            {
                kind: "category",
                name: "Text",
                colour: "#5da48c",
                contents: [
                    {
                        'kind': 'block',
                        'type': 'text'
                    },
                ]
            },
            {
                kind: "category",
                name: "Math",
                colour: "#5CA65C",
                contents: [
                    {
                        kind: "block",
                        type: "math_round",
                    },
                    {
                        kind: "block",
                        type: "math_number",
                    },
                ],
            },
            {
                kind: "category",
                name: "Custom",
                colour: "#5CA699",
                contents: [
                    {
                        kind: "block",
                        type: "new_boundary_function",
                    },
                    {
                        kind: "block",
                        type: "return",
                    },
                ],
            },
        ],
    };

    function toCode(workspace: Blockly.WorkspaceSvg) {
        const generator = new Blockly.Generator("maker");
        generator.forBlock["math_number"] = function (block, generator) {
            return [String(block.getFieldValue("NUM")), 0]
        }

        generator.forBlock["logic_null"] = function () {
            return ["null", 0];
        }

        generator.forBlock["logic_compare"] = function (block) {
            const OPERATORS = {
                'EQ': '==',
                'NEQ': '!=',
                'LT': '<',
                'LTE': '<=',
                'GT': '>',
                'GTE': '>=',
            };
            type OperatorOption = keyof typeof OPERATORS;
            const operator = OPERATORS[block.getFieldValue('OP') as OperatorOption];
            const argument0 = generator.valueToCode(block, 'A', 0) || '0';
            const argument1 = generator.valueToCode(block, 'B', 0) || '0';
            const code = argument0 + ' ' + operator + ' ' + argument1;
            return [code, 0];
        }

        generator.forBlock["controls_if"] = function (block) {
            let code = "";
            let n = 0;

            do {
                // Get the condition
                const conditionCode =
                    generator.valueToCode(block, 'IF' + n, 0) || 'false';
                let branchCode = generator.statementToCode(block, 'DO' + n);
                if (generator.STATEMENT_SUFFIX) {
                    branchCode =
                        generator.prefixLines(
                            generator.injectId(generator.STATEMENT_SUFFIX, block),
                            generator.INDENT,
                        ) + branchCode;
                }
                code +=
                    (n > 0 ? ' else ' : '') +
                    'if (' +
                    conditionCode +
                    ') do\n' +
                    branchCode +
                    'end';
                n++;
            } while (block.getInput('IF' + n));

            if (block.getInput('ELSE') || generator.STATEMENT_SUFFIX) {
                let branchCode = generator.statementToCode(block, 'ELSE');
                if (generator.STATEMENT_SUFFIX) {
                    branchCode =
                        generator.prefixLines(
                            generator.injectId(generator.STATEMENT_SUFFIX, block),
                            generator.INDENT,
                        ) + branchCode;
                }
                code += ' else do\n' + branchCode + 'end';
            }
            console.log(code);

            return code + '\n';

        }

        generator.forBlock["main"] = function (block) {
            let b = generator.statementToCode(block, "do");
            return `def main() do\n${b}end\nmain()`;
        }

        let result = generator.workspaceToCode(workspace);
        setTest(result);
    }

    return (
        <>
            <HeaderText>Create a new bot</HeaderText>
            <label>Name:</label>
            <Input className="jumbo" ref={nameRef} />
            <label>Token:</label>
            <Input className="jumbo" ref={tokenRef} />
            <Button className="jumbo" onClick={createBot}>Create</Button>

            <div className="side-by-side">
                <BlocklyWorkspace
                    className="blockly-container"
                    workspaceConfiguration={{}}
                    initialXml={initialXml}
                    toolboxConfiguration={toolboxCategories}
                    onWorkspaceChange={toCode}
                />
                <pre style={{ whiteSpace: "pre-line" }}>
                    {test}
                </pre>
            </div>
        </>
    );
}