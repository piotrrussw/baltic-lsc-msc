import { Link } from "react-router-dom";

export function Nav() {
  return (
    <nav>
      <div className="nav-container">
        <div className="nav-logo">
          <Link to="/">
            <div className="row">
              <div className="col col-sm-6">Baltic LSC</div>
              <div className="col col-sm-6">
                <img src="/logo2.jpg" width={150} height="100%" />
              </div>
            </div>
          </Link>
        </div>
        <ul className="nav-links">
          <li>
            <Link to="/">Store</Link>
          </li>
          <li>
            <Link to="/shelf">Shelf</Link>
          </li>
        </ul>
      </div>
    </nav>
  );
}
