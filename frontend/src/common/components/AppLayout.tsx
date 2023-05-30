import React from "react";
import { Outlet } from "react-router-dom";

export default function AppLayout() {
  return (
    <div>
      {/* nav */}
      <Outlet />
      {/* footer */}
    </div>
  );
}
