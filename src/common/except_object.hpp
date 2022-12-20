#pragma once

#include "common/object.hpp"
#include "common/utils.hpp"

namespace common {

class ExceptObject : public Object {
public:
  using Object::Object;
  ExceptObject(const ExceptObject& o) : Object(o){};
  ExceptObject(ExceptObject&& o) : Object(std::move(o)){};
};

} // namespace common