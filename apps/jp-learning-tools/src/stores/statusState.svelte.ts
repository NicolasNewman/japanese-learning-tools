type Status = {
  manifestStatus: "idle" | "loading" | "success" | "error";
  manifestError?: string;
};

export const statusState: Status = $state({
  manifestStatus: "idle",
});
