import { Button } from "../../components/Button";
import { Input } from "../../components/Input";
import styled from "styled-components";

const Conainer = styled.div`
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;

  > div {
    display: flex;
    flex-direction: column;
    gap: 1em;
  }
`;

export const LoginRoute: React.FC = () => {
  return (
    <Conainer>
      <div>
        <Input placeholder="Username" />
        <Input placeholder="Password" />
        <Button>Login</Button>
      </div>
    </Conainer>
  );
};
