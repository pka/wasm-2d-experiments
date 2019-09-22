import { RenderEnv } from '../crate/Cargo.toml'
import createFps from 'fps-indicator'

const renderEnv = RenderEnv.new();

let fps = createFps();

const renderLoop = () => {
    renderEnv.render();

    requestAnimationFrame(renderLoop);
};

renderLoop();
