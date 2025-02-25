import { type RouteConfig, index } from "@react-router/dev/routes";

export default [
    index("routes/home.tsx"),

    { path: "battle", file: "routes/battle.tsx", },

    { path: "fishing", file: "routes/fishing.tsx", },

    { path: "training", file: "routes/training.tsx", },

] satisfies RouteConfig;
