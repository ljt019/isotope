import {
  HashRouter as Router,
  Routes as RoutePrimitive,
  Route,
} from "react-router-dom";
import TitleBar from "@/components/titlebar";
import { Index } from "./screens/index";

type Route = {
  path: string;
  element: JSX.Element;
};

const routes: Route[] = [
  {
    path: "/",
    element: <Index />,
  },
];

function Routes({ routes }: { routes: Route[] }) {
  return (
    <RoutePrimitive>
      {routes.map((route) => (
        <Route
          key={route.path}
          path={route.path}
          element={<Layout>{route.element}</Layout>}
        />
      ))}
    </RoutePrimitive>
  );
}

function Layout({ children }: { children: JSX.Element }) {
  return (
    <>
      <TitleBar />
      <div className="pt-6 overflow-hidden h-screen">{children}</div>
    </>
  );
}

function App() {
  return (
    <Router>
      <Routes routes={routes} />
    </Router>
  );
}

export default App;
