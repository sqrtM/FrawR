import React, { memo } from "react";
import { Entity, Point, Tile } from "rustlib";

type TileSpanProps = {
  tile: [Point, Tile],
  entities: Entity | false
  player: Entity | false
}

const TileSpan = memo(function TileSpan(props: TileSpanProps): React.JSX.Element {

  //console.log("rereder chek" + props.tile[0].x + "-" + props.tile[0].y)

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
