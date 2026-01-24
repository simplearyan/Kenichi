// Utility: Snapping Math
export const SNAP_THRESHOLD_PX = 10;

export function calculateSnap(position: number, guide: number): number {
    if (Math.abs(position - guide) < SNAP_THRESHOLD_PX) {
        return guide;
    }
    return position;
}
