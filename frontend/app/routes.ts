import { type RouteConfig, index } from "@react-router/dev/routes";

export default [
    index("routes/home.tsx"),

    { path: "battle", file: "routes/battle.tsx", },

    { path: "fishing", file: "routes/fishing.tsx", },

    { path: "wearing", file: "routes/wearing.tsx", },

    {path: "signup", file: "routes/signup.tsx", },

    {path: "login", file: "routes/login.tsx", },

    {path: "logout", file: "routes/logout.tsx", },

    {path: "battle-lobby", file: "routes/BattleLobby.tsx", },


] satisfies RouteConfig;
