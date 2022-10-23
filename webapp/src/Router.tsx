import { Navigate, Route, Routes, useNavigate } from "react-router-dom";

import { EditRoute } from "./routes/edit/Edit";
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
      <Route path="/new" element={<EditRoute isNew />} />
      <Route path="/:id" element={<LinkRoute />} />
      <Route path="/:id/edit" element={<EditRoute />} />
      <Route path="*" element={<Navigate to="/" />} />
    </Routes>
  );
};
