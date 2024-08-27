import React, { HTMLAttributes, ReactNode } from "react";

export default function Button({ children, type, ...rest }: { children: ReactNode, type?: "normal" | "error" | "success", disabled?: boolean } & HTMLAttributes<HTMLButtonElement>) {
    return (
        <button {...rest} className={`button ${rest.className} ${type || ""}`}>
            {children}
        </button>
    );
}