import {
  HashRouter as Router,
  Routes as RoutePrimitive,
  Route,
} from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import Sidebar from "@/components/sidebar/sidebar";

import TitleBar from "@/components/titlebar";

import { Index } from "./screens/index";

type Route = {
  path: string;
  element: React.ReactNode;
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

function Layout({ children }: { children: React.ReactNode }) {
  return (
    <Sidebar>
      <TitleBar />
      {children}
    </Sidebar>
  );
}

export const queryClient = new QueryClient();

function App() {
  return (
    <Router>
      <QueryClientProvider client={queryClient}>
        <Routes routes={routes} />
      </QueryClientProvider>
    </Router>
  );
}

export default App;
