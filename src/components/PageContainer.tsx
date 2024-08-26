import React from "react";
import { ReactNode } from "react";

export default function PageContainer(props: { children: ReactNode }) {
    return (
        <div className="page-container">
            {props.children}
        </div>
    );
}