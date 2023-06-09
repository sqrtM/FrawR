import React, { memo } from "react";
import { Entity, Tile } from "rustlib";

type TileSpanProps = {
  tile: [[x: number, y: number], Tile],
  entities: Entity | false
  player: Entity | false
}

const TileSpan = memo(function TileSpan(props: TileSpanProps): React.JSX.Element {

  //console.log("rereder chek" + props.tile[0][0] + "-" + props.tile[0][1])

  return (
    <>
      {
        selectChar(props)
      }
    </>
  )
}, areEqual)

function selectChar(props: TileSpanProps): string {
  return props.player
    ? props.player.char
    : props.entities
      ? props.entities.char
      : props.tile[1].char
}

function areEqual(a: TileSpanProps, b: TileSpanProps) {
  return selectChar(a) === selectChar(b)
}

export default TileSpan;
