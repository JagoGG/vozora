// Wordmark oficial de Vozora (PNG original de marca). En tema oscuro se
// aplica el filtro .brand-adaptive (App.css) para que el navy no se pierda
// sobre fondo oscuro.
import logo from "../../assets/vozora-logo.png";

const VozoraTextLogo = ({
  width,
  height,
  className,
}: {
  width?: number;
  height?: number;
  className?: string;
}) => {
  return (
    <img
      src={logo}
      alt="Vozora"
      width={width}
      height={height}
      className={`brand-adaptive select-none ${className ?? ""}`}
      draggable={false}
    />
  );
};

export default VozoraTextLogo;
