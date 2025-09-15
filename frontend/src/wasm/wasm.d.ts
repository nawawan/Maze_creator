/* tslint:disable */
/* eslint-disable */
export function start(): void;
export function draw_maze(from_x: number, from_y: number, row: number, col: number, space: number, maze: MazeType): void;
export enum MazeType {
  Random = 0,
  SingleStroke = 1,
}
