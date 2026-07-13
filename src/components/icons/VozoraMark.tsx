// Isotipo oficial de Vozora (PNG original de marca): onda de voz + V + cursor.
// En tema oscuro se aplica .brand-adaptive (App.css) para mantener contraste.
import isotipo from "../../assets/vozora-isotipo.png";

const VozoraMark = ({
  width,
  height,
  className,
}: {
  width?: number | string;
  height?: number | string;
  className?: string;
}) => (
  <img
    src={isotipo}
    alt=""
    width={width || 128}
    height={height || width || 128}
    className={`brand-adaptive select-none object-contain ${className ?? ""}`}
    draggable={false}
  />
);

export default VozoraMark;
