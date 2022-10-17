import { Navigate, Route, Routes, useNavigate } from "react-router-dom";

import { InitRoute } from "./routes/init/Init";
import { LinkRoute } from "./routes/link/Link";
import { LoginRoute } from "./routes/login/Login";
import { MainRoute } from "./routes/main/Main";
import { useApi } from "./hooks/useApi";
import { useEffectAsync } from "./hooks/useEffectAsync";

type Props = {};

export const Router: React.FC<Props> = ({}) => {
  const fetch = useApi();
  const nav = useNavigate();

  useEffectAsync(async () => {
    const { initialized } = await fetch((c) => c.authCheckInit());
    if (!initialized) return nav("/init");
    await fetch((c) => c.authCheck());
  }, []);

  return (
    <Routes>
      <Route index element={<MainRoute />} />
      <Route path="/login" element={<LoginRoute />} />
      <Route path="/init" element={<InitRoute />} />
      <Route path="/:ident" element={<LinkRoute />} />
      <Route path="*" element={<Navigate to="/" />} />
    </Routes>
  );
};
