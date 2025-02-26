import { type RouteConfig, index } from "@react-router/dev/routes";

export default [
    index("routes/home.tsx"),

    { path: "battle", file: "routes/battle.tsx", },

    { path: "fishing", file: "routes/fishing.tsx", },

    { path: "wearing", file: "routes/wearing.tsx", },


] satisfies RouteConfig;
