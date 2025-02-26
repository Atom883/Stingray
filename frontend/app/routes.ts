import { type RouteConfig, index } from "@react-router/dev/routes";

export default [
    index("routes/home.tsx"),

    { path: "battle", file: "routes/battle.tsx", },

    { path: "fishing", file: "routes/fishing.tsx", },

    { path: "custom", file: "routes/custom.tsx", },


] satisfies RouteConfig;
