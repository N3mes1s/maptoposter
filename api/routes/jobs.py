"""Job status and SSE streaming routes."""

import asyncio
import json
from fastapi import APIRouter, HTTPException
from fastapi.responses import StreamingResponse

from api.routes.posters import jobs

router = APIRouter()


@router.get("/{job_id}/stream")
async def stream_job_progress(job_id: str):
    """Server-Sent Events stream for job progress updates."""
    if job_id not in jobs:
        raise HTTPException(status_code=404, detail="Job not found")

    async def event_generator():
        last_progress = -1
        last_status = None

        while True:
            if job_id not in jobs:
                yield f"event: error\ndata: {json.dumps({'message': 'Job not found'})}\n\n"
                break

            job = jobs[job_id]
            current_progress = job["progress"]
            current_status = job["status"]

            # Only send update if something changed
            if current_progress != last_progress or current_status != last_status:
                last_progress = current_progress
                last_status = current_status

                if current_status == "completed":
                    yield f"event: completed\ndata: {json.dumps({'download_url': job['download_url']})}\n\n"
                    break
                elif current_status == "failed":
                    yield f"event: error\ndata: {json.dumps({'message': job.get('error', 'Unknown error')})}\n\n"
                    break
                else:
                    yield f"event: progress\ndata: {json.dumps({'step': job['current_step'], 'percent': int(current_progress * 100), 'message': job['message']})}\n\n"

            await asyncio.sleep(0.5)

    return StreamingResponse(
        event_generator(),
        media_type="text/event-stream",
        headers={
            "Cache-Control": "no-cache",
            "Connection": "keep-alive",
            "X-Accel-Buffering": "no"
        }
    )
