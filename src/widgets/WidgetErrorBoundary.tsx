import { Component, type ReactNode } from "react";

interface Props {
  children: ReactNode;
  instance: string;
}

interface State {
  error: Error | null;
}

export class WidgetErrorBoundary extends Component<Props, State> {
  state: State = { error: null };

  static getDerivedStateFromError(error: Error) {
    return { error };
  }

  render() {
    if (this.state.error) {
      return (
        <div className="widget-error" data-instance={this.props.instance}>
          <p>Widget unavailable</p>
        </div>
      );
    }
    return this.props.children;
  }
}
