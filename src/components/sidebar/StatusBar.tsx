import React from "react"
import { StatusBar as StatusBarInfo } from "rustlib/rustlib"
import styles from "../../styles/StatusBar.module.scss"

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

    function renderBar(bar: StatusBarInfo): string[] {
        let percent = (bar.current / bar.max) * 100;
        let p: string[] = Array(20 - (20 - Math.round(percent / 5))).fill("=");
        if (p.length < 20) {
            //if (percent % 5 !== 0) { p.push('-') }
            for (let i = (20 - (20 - p.length)); i < 20; i++) {
                p.push(".");
            }
        }
        return p;
    }

    function statusColor(bar: StatusBarInfo): string {
        let percent = (bar.current / bar.max) * 100;
        let status: string;
        if (percent === 100) {
            status = "100"
        } else if (percent >= 75) {
            status = "75"
        } else if (percent >= 50) {
            status = "50"
        } else if (percent >= 25) {
            status = "25"
        } else {
            status = "0"
        }
        return status
    }

    console.log(`${nameWithoutPrefix(props.name)}.${statusColor(props.barInfo)}`)

    return (
        <div className={styles[`${nameWithoutPrefix(props.name)}--${statusColor(props.barInfo)}`]}>
            {
                renderBar(props.barInfo)
            }
        </div>
    )
}