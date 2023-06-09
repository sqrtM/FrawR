import React, { memo } from "react";
import { Entity, Point, Tile } from "rustlib";

type TileSpanProps = {
  tile: [Point, Tile],
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
        : props.tile[1].char
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
