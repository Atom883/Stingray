export type UserState = {
  aState: AState;
  feeds: Map<Feed, number>;
}; // signup or login

export type AState = {
  name: string;
  hp: number;
  maxHp: number;
  color: string;
};

export type Feed = string; // a..z

export type EatFishResponse = {
    hp: number;
}