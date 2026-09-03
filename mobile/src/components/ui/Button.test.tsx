import { fireEvent, render, screen } from "@testing-library/react-native";

import { Button } from "./Button";

describe("<Button />", () => {
  it("renders its label and fires onPress", () => {
    const onPress = jest.fn();
    render(<Button label="Log in" onPress={onPress} />);

    fireEvent.press(screen.getByText("Log in"));
    expect(onPress).toHaveBeenCalledTimes(1);
  });

  it("does not fire onPress while loading", () => {
    const onPress = jest.fn();
    render(<Button label="Log in" loading onPress={onPress} />);

    expect(screen.queryByText("Log in")).toBeNull();
  });
});
