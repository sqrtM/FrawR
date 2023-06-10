import React from "react"
import { StatusBar as StatusBarInfo } from "rustlib/rustlib"

interface StatusBarProps {
    name: string,
    barInfo: StatusBarInfo
}

export default function StatusBar(props: StatusBarProps): React.JSX.Element {

    function getPrefix(string: string) {
        let prefix = ''
        let ind = string.indexOf(':');
        if (ind > 0) {
            prefix = string.slice(0, ind)
        }
        return prefix
    }

    function nameWithoutPrefix(string: string) {
        return string.slice(string.indexOf(':') + 1)
    }

    return (
        <div>
            {
                
            }
        </div>
    )
}