import React from "react";
import { ReactNode } from "react";

export default function HeaderText(props: { children: ReactNode }) {
    return (
        <label role="heading" className="header">
            {props.children}
        </label>
    );
}