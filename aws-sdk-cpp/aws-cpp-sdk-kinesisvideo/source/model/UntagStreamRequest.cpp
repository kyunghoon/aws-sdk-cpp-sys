﻿/**
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

#include <aws/kinesisvideo/model/UntagStreamRequest.h>
#include <aws/core/utils/json/JsonSerializer.h>

#include <utility>

using namespace Aws::KinesisVideo::Model;
using namespace Aws::Utils::Json;
using namespace Aws::Utils;

UntagStreamRequest::UntagStreamRequest() : 
    m_streamARNHasBeenSet(false),
    m_streamNameHasBeenSet(false),
    m_tagKeyListHasBeenSet(false)
{
}

Aws::String UntagStreamRequest::SerializePayload() const
{
  JsonValue payload;

  if(m_streamARNHasBeenSet)
  {
   payload.WithString("StreamARN", m_streamARN);

  }

  if(m_streamNameHasBeenSet)
  {
   payload.WithString("StreamName", m_streamName);

  }

  if(m_tagKeyListHasBeenSet)
  {
   Array<JsonValue> tagKeyListJsonList(m_tagKeyList.size());
   for(unsigned tagKeyListIndex = 0; tagKeyListIndex < tagKeyListJsonList.GetLength(); ++tagKeyListIndex)
   {
     tagKeyListJsonList[tagKeyListIndex].AsString(m_tagKeyList[tagKeyListIndex]);
   }
   payload.WithArray("TagKeyList", std::move(tagKeyListJsonList));

  }

  return payload.View().WriteReadable();
}



