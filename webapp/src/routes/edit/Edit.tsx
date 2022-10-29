import styled, { useTheme } from "styled-components";
import { useEffect, useReducer } from "react";
import { useNavigate, useParams } from "react-router-dom";

import { ReactComponent as BackIcon } from "../../assets/back.svg";
import { Floatbar } from "../../components/Floatbar";
import { Input } from "../../components/Input";
import { Link } from "../../lib/tenso";
import { ReactComponent as SaveIcon } from "../../assets/save.svg";
import { Switch } from "../../components/Slider";
import { useApi } from "../../hooks/useApi";

type Props = {
  isNew?: boolean;
};

const Container = styled.div`
  padding: 1em;
  max-width: 50em;
  width: 100%;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 1.5em;

  > h2 {
    margin: 0;
  }

  label,
  input {
    display: block;
    width: 100%;
  }

  > section > label {
    margin-bottom: 0.5em;
    font-weight: 300;
    font-size: 1.4em;
    opacity: 0.75;
  }
`;

const linkReducer = (
  state: Link,
  [type, payload]:
    | ["set", Partial<Link>]
    | ["set_ident" | "set_destination", string]
    | ["set_enabled" | "set_permanent", boolean]
) => {
  switch (type) {
    case "set":
      return { ...state, ...payload };
    case "set_ident":
      return { ...state, ident: payload };
    case "set_destination":
      return { ...state, destination: payload };
    case "set_enabled":
      return { ...state, enabled: payload };
    case "set_permanent":
      return { ...state, permanent_redirect: payload };
    default:
      return payload;
  }
};

export const EditRoute: React.FC<Props> = ({ isNew = false }) => {
  const theme = useTheme();
  const { id } = useParams();
  const fetch = useApi();
  const nav = useNavigate();
  const [link, dispatch] = useReducer(linkReducer, {
    enabled: true,
    permanent_redirect: true,
  } as Link);

  const save = () => {
    if (isNew) {
      fetch((c) => c.linkCreate(link)).then(() => nav(-1));
    } else {
      fetch((c) => c.linkUpdate(link.id, link));
    }
  };

  useEffect(() => {
    if (!id || isNew) return;
    fetch((c) => c.link(id)).then((link) => dispatch(["set", link]));
  }, [id]);

  return (
    <>
      <Container>
        <h2>{(isNew && <>Create Link</>) || <>Edit Link</>}</h2>
        <section>
          <label htmlFor="i-ident">Ident</label>
          <Input
            id="i-ident"
            value={link.ident}
            onInput={(e) => dispatch(["set_ident", e.currentTarget.value])}
          />
        </section>
        <section>
          <label htmlFor="i-destination">Destination</label>
          <Input
            id="i-destination"
            value={link.destination}
            onInput={(e) =>
              dispatch(["set_destination", e.currentTarget.value])
            }
          />
        </section>
        <section>
          <Switch
            enabled={link.enabled}
            onChange={(v) => dispatch(["set_enabled", v])}
            labelAfter="Enabled"
          />
        </section>
        <section>
          <Switch
            enabled={link.permanent_redirect}
            onChange={(v) => dispatch(["set_permanent", v])}
            labelAfter="Permanent Redirect"
          />
        </section>
      </Container>
      <Floatbar
        buttons={[
          {
            content: (
              <>
                <BackIcon />
                Back
              </>
            ),
            color: theme.orange,
            action: () => nav(-1),
          },
          {
            content: (
              <>
                <SaveIcon />
                {isNew ? "Create" : "Save"}
              </>
            ),
            color: theme.green,
            action: save,
            disabled: !link.ident || !link.destination,
          },
        ]}
      />
    </>
  );
};
