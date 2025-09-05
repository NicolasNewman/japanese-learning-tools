export type AlertType = "info" | "success" | "warning" | "error";
type AlertState = {
  alert: {
    alertTitle: string;
    alertMessage: string;
    alertType: AlertType;
  } | null;
};

export const alertState: AlertState = $state({
  alert: null,
});
