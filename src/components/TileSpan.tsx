import React, { memo } from "react";
import { Entity, Tile } from "rustlib";

type TileSpanProps = {
  tile: Tile,
  entities: Entity | false
  player: Entity | false
}

const TileSpan = memo(function TileSpan(props: TileSpanProps): React.JSX.Element {

  //console.log("rereder chek" + props.tile.location.x + "-" + props.tile.location.y)

  function selectChar(): string {
    return props.player
      ? props.player.char
      : props.entities
        ? props.entities.char
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
  return a.entities === b.entities
    && a.entities
    && b.entities
    && a.entities.char === b.entities.char
    && a.player === b.player;
}

export default TileSpan;
