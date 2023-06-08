import React, { memo } from "react";
import { Entity, Tile } from "rustlib";

type TileSpanProps = {
  tile: Tile,
  entities: Entity[]
  player: Entity | false
}

const TileSpan = memo(function TileSpan(props: TileSpanProps): React.JSX.Element {

  //console.log("rereder chek" + props.tile.location.x + "-" + props.tile.location.y)

  function selectChar(): string {
    return props.player
      ? props.player.char
      : props.entities.length > 0
        ? props.entities[0].char
        : props.tile.char
  }

  return (
    <>
      {
        selectChar()
      }
    </>
  )
}, areEqual)

function areEqual(a: TileSpanProps, b: TileSpanProps) {
  return a.entities.length === b.entities.length
    && a.entities.every((val, index) => val === b.entities[index])
    && a.player === b.player;
}

export default TileSpan;
