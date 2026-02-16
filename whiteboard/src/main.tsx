import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import WhiteboardApp from './App'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <WhiteboardApp />
  </StrictMode>,
)
