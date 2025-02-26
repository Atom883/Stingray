import { type RouteConfig, index } from "@react-router/dev/routes";

export default [
    index("routes/home.tsx"),

    { path: "battle", file: "routes/battle.tsx", },

    { path: "fishing", file: "routes/fishing.tsx", },

    { path: "custom", file: "routes/custom.tsx", },

    {path: "signup", file: "routes/signup.tsx", },

    {path: "login", file: "routes/login.tsx", },

    {path: "logout", file: "routes/logout.tsx", },


] satisfies RouteConfig;
